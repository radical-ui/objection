import {
	Color,
	DropdownOption,
	FlatLoader,
	getColor,
	GlobalCss,
	isOk,
	makeDebounce,
	React,
	TextInput,
	TextInputRole,
	TextInputUpdate,
	useDisabledContext,
	useDispatcher,
	useUpdates,
} from 'runtime'
import { IconRender } from '../icon/mod.tsx'
import { JustTheCheckbox } from '../checkbox_input/mod.tsx'

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

interface InputNotice {
	isError: boolean
	message: string
}

interface SelectedOption {
	id: string
	title: string
}

export function TextInputRender(props: TextInput) {
	const initialOptions = props.initial_selected_options && props.initial_dropdown_options
		? props.initial_dropdown_options.filter((option) => props.initial_selected_options!.includes(option.id)).map((option) => ({
			id: option.id,
			title: option.title,
		}))
		: []

	const hasActionStopper = useDisabledContext()
	const { isLoading: changeIsLoading, dispatch: dispatchChange } = useDispatcher(props.change_action ?? null)
	const { isLoading: blurIsLoading, dispatch: dispatchBlur } = useDispatcher(props.blur_action ?? null)
	const { isLoading: dropdownSelectionIsLoading, dispatch: dispatchDropdownSelection } = useDispatcher(props.option_selected_action ?? null)
	const { isLoading: submitIsLoading, dispatch: dispatchSubmit } = useDispatcher(props.submit_action ?? null)
	const update = useUpdates<TextInputUpdate>(props.update_hook ?? null)
	const [text, setText] = React.useState(props.initial_value ?? '')
	const [conceal, setConceal] = React.useState(props.role === 'Password')
	const [dropdownOptions, setDropdownOptions] = React.useState(props.initial_dropdown_options || [])
	const [isFocused, setIsFocused] = React.useState(false)
	const [notice, setNotice] = React.useState<InputNotice | null>(null)
	const [selectedOptions, setSelectedOptions] = React.useState<SelectedOption[]>(initialOptions)
	const [activeDropdownOptionIndex, setActiveDropdownOptionIndex] = React.useState<number | null>(null)
	const inputElement = React.useRef<HTMLInputElement | null>(null)

	React.useEffect(() => {
		setConceal(props.role === 'Password')
	}, [props.role])

	React.useEffect(() => {
		if (!update) return
		if (update.type === 'SetValidity') setNotice({ isError: update.content.validity === 'Invalid', message: update.content.message })
		if (update.type === 'SetDropdownOptions') setDropdownOptions(update.content.options)
	}, [update])

	const hasNoNotableActions = !isOk(props.option_selected_action) && !isOk(props.blur_action) && !isOk(props.change_action)
	const isDisabled = hasNoNotableActions || submitIsLoading || hasActionStopper

	// if the server doesn't listen for dropdown selection actions, the text field is considered free.
	const isFreeText = !isOk(props.option_selected_action)

	const isLoading = changeIsLoading || blurIsLoading || dropdownSelectionIsLoading
	const showDropdown = isFocused && dropdownOptions.length > 0
	const focusColor: Color = { type: notice ? notice.isError ? 'Danger' : 'Success' : 'Primary', opacity: 100 }
	const normalColor = notice ? notice.isError ? 'danger' : 'success' : 'fore'
	const isActive = text.trim().length > 0 || selectedOptions.length > 0
	const labelBaseClasses = ['scale-75', 'translate-y-[-25%]', 'translate-x-[-12.5%]', `text-${getColor(focusColor)}`]
	const labelClasses = isActive ? labelBaseClasses : labelBaseClasses.map((c) => `group-focus-within:${c}`)
	const trailingIcon = props.trailing_icon ?? getTrailingIcon(props.role, conceal)
	const swapIcon = props.role === 'Password' && !props.trailing_icon

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

		// if a change action is supplied, we assume that the server will do option filtering
		if (!skipDropdownRefresh && !isOk(props.change_action) && props.initial_dropdown_options?.length) {
			setDropdownOptions(filterOptionsWithSearch(text, props.initial_dropdown_options))
		}
	}

	const selectDropdownOption = (item: DropdownOption) => {
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
		if (isOk(props.option_selected_action) && !props.multiple) {
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
			if (dropdownOptions[nextDropdownIndex].is_disabled) {
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
			if (dropdownOptions[previousDropdownIndex].is_disabled) {
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

	const iconColor: Color = { type: 'Fore', opacity: 60 }
	const leadingIconNode = props.leading_icon && <IconRender name={props.leading_icon} size={20} color={iconColor} />
	const trailingIconNode = trailingIcon && <IconRender name={trailingIcon} size={20} color={iconColor} />
	const reducedTrailingIconNode = swapIcon ? <Clicker onClicked={() => setConceal(!conceal)}>{trailingIconNode}</Clicker> : trailingIconNode

	return (
		<label
			class={`
				relative group flex gap-10 items-center cursor-text w-full border border-${normalColor}-10
				focus-within:ring-4 ring-${getColor(focusColor)}-40 focus-within:bg-${getColor(focusColor)}-10
				bg-${normalColor}-5 rounded px-14 transition-colors
				${isDisabled ? 'opacity-50' : ''}
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
						defaultValue={props.initial_value ?? ''}
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
						inputMode={getInputMode(props.role)}
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
					{notice && (
						<>
							<div>•</div>
							<div>{notice.message}</div>
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
								showCheckbox={props.multiple}
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
			disabled={props.is_disabled}
			class={`
				flex gap-15 items-center px-14
				${props.is_disabled ? 'cursor-not-allowed opacity-50' : props.isActive ? 'bg-primary-10' : 'bg-transparent hover:bg-fore-5'}
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
				<IconRender color={{ type: 'Fore', opacity: 80 }} name='mdi-close-thick' size={15} />
			</div>
			<div>{props.title}</div>
		</div>
	)
}
