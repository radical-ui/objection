import { Color } from './theme.tsx'
import { EventKey, React } from './deps.ts'
import { useDispatcher } from './event.tsx'
import { SkeletonBlock, useSkeletonDetection } from './skeleton.tsx'
import { getColor } from './utils.ts'
import { FlatLoader } from './flat_loader.tsx'

/**
 * A simple label
 *
 * **Example**
 *
 * ```rust #[derive(Serialize, Deserialize, HasActionKey)] enum Event { Foo }
 *
 * Flex::new(FlexKind::Column) .gap(5) .justify(FlexJustify::Center) .align(FlexAlign::Center) .auto_item(Label::new("Some Label")) .auto_item(Label::new("Italic").italic()) .auto_item(Label::new("Bold").bold()) .auto_item(Label::new("Another Color").color(Color::Primary(100))) .auto_item(Label::new("This one is editable").edit_event(Event::Foo).color(Color::Primary(100))) .auto_item( Flex::new(FlexKind::Row) .auto_item(Label::new("And so is this").edit_event(Event::Foo)) .auto_item(Label::new("And this too (with a placeholder)").edit_event(Event::Foo).placeholder("This is the placeholder!!!! It is pretty long.")) ) ```
 *
 * @component
 */
export interface Label {
	text: string

	color?: Color
	editEvent?: EventKey<string>
	bold?: boolean
	italic?: boolean
	placeholder?: string
}

export function LabelRender(props: Label) {
	const divRef = React.useRef<HTMLDivElement | null>(null)
	const inputRef = React.useRef<HTMLTextAreaElement | null>(null)

	const [isEditing, setIsEditing] = React.useState(false)
	const [value, setValue] = React.useState(props.text || '')
	const [dirtyValue, setDirtyValue] = React.useState(props.text || '')
	const { dispatch, isDisabled, isLoading } = useDispatcher(props.editEvent ?? null)

	const isSkeleton = useSkeletonDetection()
	const showPlaceholder = dirtyValue.length === 0
	const hideStaticLabel = isSkeleton || isEditing && !showPlaceholder
	const placeholder = props.placeholder ?? 'No content'

	const canEdit = !!props.editEvent && !isDisabled
	const isItalic = props.italic || showPlaceholder && !isEditing
	const color = showPlaceholder ? { type: props.color?.type || 'Fore', def: 30 } : props.color ?? { type: 'Fore', def: 80 }

	const textStyleClasses = `
		${props.bold ? 'font-semibold' : ''}
		${isItalic ? 'italic' : ''}
	`
	const styleClasses = `
		cursor-text
		${textStyleClasses}
		${props.color ? `text-${getColor(color)}` : ''}
		${canEdit ? 'px-5 py-2 border  transition-colors rounded' : ''}
	`

	React.useEffect(() => {
		setDirtyValue(props.text || '')
	}, [props.text])

	React.useEffect(() => {
		const input = inputRef.current

		if (input && document.activeElement !== input) {
			input.focus()
			input.setSelectionRange(0, input.value.length, 'forward')
		}
	})

	const finishEdits = () => {
		setIsEditing(false)

		if (dirtyValue === value) return

		setValue(dirtyValue)
		dispatch(dirtyValue)
	}

	const cancelEdits = () => {
		setIsEditing(false)
		setDirtyValue(value)
	}

	return (
		<div
			class={`relative`}
		>
			<div
				ref={divRef}
				class={`
					${styleClasses}
					${hideStaticLabel ? `def-0 pointer-events-none` : ''}
					${canEdit ? 'border-transparent hover:border-fore-10' : ''}
					min-h-24 min-w-1
					whitespace-pre-wrap
				`}
				onClick={() => {
					if (canEdit) setIsEditing(true)
				}}
				style={{ minWidth: isEditing ? '100px' : '1px', minHeight: '24px', overflowWrap: 'anywhere' }}
			>
				{showPlaceholder ? placeholder : dirtyValue}
			</div>

			{isEditing && (
				<div class='absolute inset-0'>
					<textarea
						type='text'
						class={`${styleClasses} border-fore-10 w-full h-full overflow-hidden outline-none ring-4 ring-primary-40 bg-transparent resize-none`}
						value={dirtyValue}
						ref={inputRef}
						onLoad={(event) => event.currentTarget.focus()}
						onChange={(event) => {
							if (event.currentTarget.value.includes('\n')) {
								event.currentTarget.value = event.currentTarget.value.replaceAll('\n', ' ')
							}
							setDirtyValue(event.currentTarget.value)
						}}
						onKeyDown={(event) => {
							if (event.key === 'Enter' || event.key === 'Return') {
								event.preventDefault()
								finishEdits()
							}

							if (event.key === 'Escape') {
								event.preventDefault()
								cancelEdits()
							}
						}}
						onBlur={() => finishEdits()}
					/>
				</div>
			)}

			{isSkeleton && (
				<div class='absolute inset-0'>
					<SkeletonBlock width={null} height={null} rounding='slight' />
				</div>
			)}

			{isLoading && (
				<div class='absolute left-5 right-5 bottom-0 height-2 overflow-hidden'>
					<FlatLoader color={{ type: 'Primary', def: 100 }} size={2} />
				</div>
			)}
		</div>
	)
}
