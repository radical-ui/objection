import { useDispatcher } from './event.tsx'
import { Color } from './theme.tsx'
import { ActionKey, EventKey, React } from './deps.ts'
import { IconRender } from './icon.tsx'
import { getColor, isOk, makeDebounce } from './utils.ts'
import { FlatLoader } from './flat_loader.tsx'
import { JustTheCheckbox } from './checkbox_input.tsx'
import { useAction } from './action.tsx'

const getTrailingIcon = (role: TextInputRole, conceal: boolean) => {
	if (role === 'Password') return conceal ? 'mdi-eye-off-outline' : 'mdi-eye-outline'

	return null
}

const getInputMode = (role: TextInputRole) => {
	if (role === 'Decimal') return 'decimal'
	if (role === 'Numeric') return 'numeric'
	if (role === 'Tel') return 'tel'
	if (role === 'Search') return 'search'
	if (role === 'Email') return 'email'
	if (role === 'Url') return 'url'

	return 'text'
}

interface SelectedOption {
	id: string
	title: string
}

export interface InputValidity {
	level: InputValidityLevel
	message?: string
}

export type InputValidityLevel = 'Valid' | 'Invalid' | 'Normal'

export type TextInputRole = 'Plain' | 'Password' | 'Email' | 'Search' | 'Url' | 'Tel' | 'Numeric' | 'Decimal'
export type TextInputHook = number

/**
 * A text input. If no change event, blur event, or dropdown selection event is supplied, the input will be disabled.
 *
 * If some initial dropdown options are supplied, but no `change_event` is supplied, the dropdown options will be sorted locally. If a `change_event` is supplied, the server is expected to send down a new list of dropdown options.
 *
 * If no `option_selection_event` is supplied, the selected dropdown options will simply replace the input value, triggering the default value update behavior.
 *
 * `allow_multiple_options` has no effect if an `option_selected_option` is not supplied. If it is, more that one option can be selected.
 *
 * @component
 */
export interface TextInput {
	blurEvent?: EventKey<string>
	changeEvent?: EventKey<string>
	initialDropdownOptions?: DropdownOption[]
	initialSelectedOptions?: string[]
	initialValue?: string
	label: string
	leadingIcon?: string
	multiple?: boolean
	optionSelectedEvent?: EventKey<string[]>
	role?: TextInputRole
	submitEvent?: EventKey<null>
	trailingIcon?: string
	defaultValidity?: InputValidity
	setOptionsAction?: ActionKey<DropdownOption[]>
	setValidityAction?: ActionKey<InputValidity>
}
export interface DropdownOption {
	description?: string
	id: string
	informative?: string
	isDisabled?: boolean
	title: string
}

export function TextInputRender(props: TextInput) {
	const role = props.role || 'Plain'
	const initialOptions = props.initialSelectedOptions && props.initialDropdownOptions
		? props.initialDropdownOptions.filter((option) => props.initialSelectedOptions!.includes(option.id)).map((option) => ({
			id: option.id,
			title: option.title,
		}))
		: []

	const { isLoading: changeIsLoading, dispatch: dispatchChange, isDisabled: changeIsDisabled } = useDispatcher(props.changeEvent ?? null)
	const { isLoading: blurIsLoading, dispatch: dispatchBlur, isDisabled: blurIsDisabled } = useDispatcher(props.blurEvent ?? null)
	const { isLoading: dropdownSelectionIsLoading, dispatch: dispatchDropdownSelection, isDisabled: dropdownSelectionIsDisabled } =
		useDispatcher(
			props.optionSelectedEvent ?? null,
		)
	const { isLoading: submitIsLoading, dispatch: dispatchSubmit, isDisabled: submitIsDisabled } = useDispatcher(props.submitEvent ?? null)
	const [text, setText] = React.useState(props.initialValue ?? '')
	const [conceal, setConceal] = React.useState(props.role === 'Password')
	const [dropdownOptions, setDropdownOptions] = React.useState(props.initialDropdownOptions || [])
	const [isFocused, setIsFocused] = React.useState(false)
	const [selectedOptions, setSelectedOptions] = React.useState<SelectedOption[]>(initialOptions)
	const [activeDropdownOptionIndex, setActiveDropdownOptionIndex] = React.useState<number | null>(null)
	const [validity, setValidity] = React.useState<InputValidity>(props.defaultValidity || { level: 'Normal' })
	const inputElement = React.useRef<HTMLInputElement | null>(null)

	useAction(props.setValidityAction || null, (validity) => setValidity(validity))
	useAction(props.setOptionsAction || null, (options) => setDropdownOptions(options))

	React.useEffect(() => {
		setConceal(props.role === 'Password')
	}, [props.role])

	const isDisabled = submitIsLoading || changeIsDisabled && blurIsDisabled && dropdownSelectionIsDisabled &&
			submitIsDisabled

	// if the server doesn't listen for dropdown selection events, the text field is considered free.
	const isFreeText = !props.optionSelectedEvent

	const isLoading = changeIsLoading || blurIsLoading || dropdownSelectionIsLoading
	const showDropdown = isFocused && dropdownOptions.length > 0
	const focusColor: Color = {
		type: validity.level === 'Invalid' ? 'Danger' : validity.level === 'Valid' ? 'Success' : 'Primary',
		def: 100,
	}
	const normalColor = validity.level === 'Invalid' ? 'danger' : validity.level === 'Valid' ? 'success' : 'fore'
	const isActive = text.trim().length > 0 || selectedOptions.length > 0
	const labelBaseClasses = ['scale-75', 'translate-y-[-25%]', 'translate-x-[-12.5%]', `text-${getColor(focusColor)}`]
	const labelClasses = isActive ? labelBaseClasses : labelBaseClasses.map((c) => `group-focus-within:${c}`)
	const trailingIcon = props.trailingIcon ?? getTrailingIcon(role, conceal)
	const swapIcon = props.role === 'Password' && !props.trailingIcon

	React.useEffect(() => {
		if (isFreeText) setActiveDropdownOptionIndex(null)
		else setActiveDropdownOptionIndex(0)
	}, [isFreeText])

	const debouncedChangeFn = React.useMemo(() => {
		return makeDebounce<string>((text) => {
			dispatchChange(text)
		}, 500)
	}, [dispatchChange])

	const handleInputChange = (text: string, skipDropdownRefresh = false) => {
		setText(text)
		debouncedChangeFn(text)

		// if a change event is supplied, we assume that the server will do option filtering
		if (!skipDropdownRefresh && !props.changeEvent && props.initialDropdownOptions?.length) {
			setDropdownOptions(filterOptionsWithSearch(text, props.initialDropdownOptions))
		}
	}

	const selectDropdownOption = (item: DropdownOption) => {
		console.log('selecting item', item)

		if (!isFreeText) {
			// if multiple options are allowed, and we selected an item that is already selected, we will unselect it
			if (props.multiple && selectedOptions.find((selectedOption) => selectedOption.id === item.id)) {
				return removeDropdownOption(item.id)
			}

			const newSelection = { id: item.id, title: item.title }
			const newSelectedOptions = props.multiple
				? [...selectedOptions.filter((option) => option.id !== item.id), newSelection]
				: [newSelection]

			setSelectedOptions(newSelectedOptions)
			dispatchDropdownSelection(newSelectedOptions.map((option) => option.id))

			if (inputElement.current) inputElement.current.value = ''
			handleInputChange('')
		}

		if (isFreeText) {
			if (inputElement.current) inputElement.current.value = item.title
			handleInputChange(item.title)
		}

		// we close blur the input when the user is not likely to select another option
		// if the text is free-flowing, the user is probably searching, and will probably want to find another result off of the current one
		if (props.optionSelectedEvent && !props.multiple) {
			if (inputElement.current) inputElement.current.blur()
		}
	}

	const removeDropdownOption = (id: string) => {
		const newSelectedOptions = selectedOptions.filter((option) => option.id !== id)
		setSelectedOptions(newSelectedOptions)
		dispatchDropdownSelection(newSelectedOptions.map((option) => option.id))
	}

	const selectActiveDropdownOption = () => {
		if (isOk(activeDropdownOptionIndex) && dropdownOptions[activeDropdownOptionIndex]) {
			selectDropdownOption(dropdownOptions[activeDropdownOptionIndex])
		}
	}

	const freeTextActivateDropdownOption = (text: string) => {
		if (inputElement.current) inputElement.current.value = text
		handleInputChange(text, true)
	}

	const activateNextDropdownOption = () => {
		let nextDropdownIndex = isOk(activeDropdownOptionIndex) ? activeDropdownOptionIndex + 1 : 0
		let iterateCount = 0

		while (true) {
			if (iterateCount > dropdownOptions.length) {
				setActiveDropdownOptionIndex(null)
				break
			}

			iterateCount++

			if (nextDropdownIndex >= dropdownOptions.length) nextDropdownIndex = 0
			if (dropdownOptions[nextDropdownIndex].isDisabled) {
				nextDropdownIndex++
				continue
			}

			setActiveDropdownOptionIndex(nextDropdownIndex)
			if (isFreeText) freeTextActivateDropdownOption(dropdownOptions[nextDropdownIndex].title)

			break
		}
	}

	const activatePreviousDropdownOption = () => {
		let previousDropdownIndex = isOk(activeDropdownOptionIndex) ? activeDropdownOptionIndex - 1 : dropdownOptions.length - 1
		let iterateCount = 0

		while (true) {
			if (iterateCount > dropdownOptions.length) {
				setActiveDropdownOptionIndex(null)
				break
			}

			iterateCount++

			if (previousDropdownIndex < 0) previousDropdownIndex = dropdownOptions.length - 1
			if (dropdownOptions[previousDropdownIndex].isDisabled) {
				previousDropdownIndex--
				continue
			}

			setActiveDropdownOptionIndex(previousDropdownIndex)
			if (isFreeText) freeTextActivateDropdownOption(dropdownOptions[previousDropdownIndex].title)
			break
		}
	}

	const removeLastSelectionOption = () => {
		if (selectedOptions.length) removeDropdownOption(selectedOptions[selectedOptions.length - 1].id)
	}

	const iconColor: Color = { type: 'Fore', def: 60 }
	const leadingIconNode = props.leadingIcon && <IconRender name={props.leadingIcon} size={20} color={iconColor} />
	const trailingIconNode = trailingIcon && <IconRender name={trailingIcon} size={20} color={iconColor} />
	const reducedTrailingIconNode = swapIcon
		? <Clicker onClicked={() => setConceal(!conceal)}>{trailingIconNode}</Clicker>
		: trailingIconNode

	return (
		<label
			class={`
				relative group flex gap-10 items-center cursor-text w-full border border-${normalColor}-10
				focus-within:ring-4 ring-${getColor(focusColor)}-40 focus-within:bg-${getColor(focusColor)}-10
				bg-${normalColor}-5 rounded px-14 transition-colors
				${isDisabled ? 'def-50' : ''}
			`}
			// we don't want clicking around to repetitively blur and refocus the input
			onMouseDown={(event) => {
				if (isFocused) event.preventDefault()
			}}
		>
			{leadingIconNode}

			<div class='flex-1 relative h-full'>
				<div class='flex gap-8 pt-18 pb-4 flex-wrap'>
					{selectedOptions.length > 0 && !props.multiple && !isFocused && (
						<div class='text-normal'>
							{selectedOptions[0].title}
						</div>
					)}

					{props.multiple &&
						selectedOptions.map((option) => (
							<SelectedOptionChip
								title={option.title}
								onClear={() => removeDropdownOption(option.id)}
								key={option.id}
							/>
						))}

					<input
						disabled={isDisabled}
						ref={inputElement}
						class='grow text-normal'
						style={{ background: 'transparent', fontSize: '1rem', outline: 'none' }}
						onInput={(event) => handleInputChange(event.currentTarget.value)}
						// This is so that mousedown events can be prevented at a higher level without affecting the default behavior of a mousedown in the input
						onMouseDown={(event) => event.stopPropagation()}
						defaultValue={props.initialValue ?? ''}
						onBlur={() => {
							setIsFocused(false)
							dispatchBlur(text)
						}}
						onKeyDown={(event) => {
							if (event.key === 'Escape') {
								event.preventDefault()
								if (inputElement.current) inputElement.current.blur()
							}

							if (event.key === 'Return' || event.key === 'Enter') {
								event.preventDefault()

								if (!isFreeText) {
									selectActiveDropdownOption()
								}

								// when ctrl/cmd is pressed we will also submit the input
								if (event.ctrlKey || event.metaKey || isFreeText) dispatchSubmit(null)
							}

							if (event.key === 'ArrowDown') {
								event.preventDefault()
								activateNextDropdownOption()
							}

							if (event.key === 'ArrowUp') {
								event.preventDefault()
								activatePreviousDropdownOption()
							}

							if (event.key === 'Delete' || event.key === 'Backspace') {
								if (event.currentTarget.selectionStart === 0 && props.multiple) removeLastSelectionOption()
							}
						}}
						onFocus={() => setIsFocused(true)}
						inputMode={getInputMode(role)}
						type={conceal ? 'password' : 'text'}
					/>
				</div>

				<div
					class={`
						absolute top-0 right-0 left-0 h-46 flex items-center transition-transform transition-colors font-semibold pointer-events-none
						${!isActive ? `text-${normalColor}-30` : ''} ${labelClasses.join(' ')}
						flex gap-5
					`}
				>
					<div>{props.label}</div>
					{validity.message && (
						<>
							<div>•</div>
							<div>{validity.message}</div>
						</>
					)}
				</div>
			</div>

			{reducedTrailingIconNode}

			{showDropdown && (
				<div
					class='absolute right-0 left-0 shadow-md bg-base rounded overflow-hidden z-30'
					style={{ top: 'calc(100% + 4px)' }}
				>
					<div class='bg-fore-5 border border-fore-10 rounded flex flex-col overflow-hidden'>
						{dropdownOptions.map((item, index) => (
							<DropdownItem
								onSelected={() => selectDropdownOption(item)}
								isActive={activeDropdownOptionIndex === index}
								showCheckbox={props.multiple || false}
								isSelected={!!selectedOptions.find((selectedOption) => selectedOption.id === item.id)}
								{...item}
							/>
						))}
					</div>
				</div>
			)}

			{isLoading && (
				<div class='absolute -bottom-2 -right-2 -left-2 h-4 bg-base overflow-hidden rounded-b'>
					<FlatLoader color={focusColor} size={4} />
				</div>
			)}
		</label>
	)
}

interface IconClickerParams {
	children: React.ReactNode
	onClicked(): void
}

function Clicker(params: IconClickerParams) {
	return (
		<button onClick={() => params.onClicked()} class='h-full flex items-center'>
			{params.children}
		</button>
	)
}

interface ExtraDropdownItemProps {
	onSelected(): void
	isActive: boolean
	showCheckbox: boolean
	isSelected: boolean
}

function DropdownItem(props: DropdownOption & ExtraDropdownItemProps) {
	return (
		<button
			disabled={props.isDisabled}
			class={`
				flex gap-15 items-center px-14
				${props.isDisabled ? 'cursor-not-allowed def-50' : props.isActive ? 'bg-primary-10' : 'bg-transparent hover:bg-fore-5'}
			`}
			onClick={(event) => {
				event.preventDefault() // we don't want to refocus the input in the case where it was
				props.onSelected()
			}}
		>
			{props.showCheckbox && <JustTheCheckbox checked={props.isSelected} />}
			<div class='py-10 w-full text-start transition-colors flex flex-col items-stretch gap-5'>
				<div class='flex gap-10 items-center'>
					<div class='flex-1'>{props.title}</div>
					{props.informative && <div class='text-sm text-fore-50'>{props.informative}</div>}
				</div>
				{props.description && <div class='text-sm text-fore-50'>{props.description}</div>}
			</div>
		</button>
	)
}

function filterOptionsWithSearch(query: string, options: DropdownOption[]): DropdownOption[] {
	const results: DropdownOption[] = []

	for (const option of options) {
		if (option.title.includes(query.trim())) results.push(option)
	}

	return results
}

interface SelectedOptionChip {
	onClear(): void
	title: string
}

function SelectedOptionChip(props: SelectedOptionChip) {
	return (
		<div class='bg-fore-10 text-fore-80 rounded text-sm flex items-center px-5 gap-2'>
			<div
				class='cursor-pointer rounded-full h-24 w-24 bg-transparent hover:bg-fore-10 transition-colors flex items-center justify-center'
				onClick={() => props.onClear()}
			>
				<IconRender color={{ type: 'Fore', def: 80 }} name='mdi-close-thick' size={15} />
			</div>
			<div>{props.title}</div>
		</div>
	)
}
