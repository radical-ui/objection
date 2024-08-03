import { useDispatcher } from './event.tsx'
import { ButtonRender } from './button.tsx'
import { ComponentRender } from './component.tsx'
import { Component, EventKey, React } from './deps.ts'
import { IconButtonRender } from './icon.tsx'
import { GlobalCss } from './utils.ts'

const css = new GlobalCss(`
	@keyframes modal-fade-in {
		from { def: 0 }
		to { def: 1 }
	}

	@keyframes modal-slide-down {
		from { top: -50px; bottom: 50px }
		to { top: 0; bottom: 0 }
	}

	.modal {
		animation: modal-fade-in 500ms;
	}
	.modal .content {
		animation: modal-slide-down 500ms
	}
`)

const getSizeClasses = (size: ModalSize) => {
	if (size === 'Large') return 'h-[90%] w-[90%]'
	if (size === 'Medium') return 'h-800 w-600'
	if (size === 'Small') return 'h-400 w-500'
}

export type ModalSize = 'Small' | 'Medium' | 'Large'

/**
 * A modal that appears over all existing content, using the context from where it is placed.
 *
 * @component
 */
export interface Modal {
	body?: Component
	cancelEvent?: EventKey<null>
	cancelEventLabel?: string
	description?: string
	finishEvent?: EventKey<null>
	finishEventLabel?: string
	size: ModalSize
	title: string
}

export function ModalRender(props: Modal) {
	const { dispatch, isDisabled } = useDispatcher(props.cancelEvent ?? null)
	css.present()

	React.useEffect(() => {
		const listener = (event: KeyboardEvent) => {
			if (!isDisabled && event.key === 'Escape') dispatch(null)
		}

		globalThis.window.addEventListener('keyup', listener)

		return () => {
			globalThis.window.removeEventListener('keyup', listener)
		}
	}, [isDisabled, props.cancelEvent])

	return (
		<div class={`fixed inset-0 modal`}>
			<div
				class='absolute inset-0 bg-fore-20'
				onClick={() => {
					if (!isDisabled) dispatch(null)
				}}
			/>

			<div class='content absolute inset-0 pointer-events-none flex items-center justify-center'>
				<div class={`pointer-events-auto rounded shadow-lg bg-base p-30 ${getSizeClasses(props.size)} flex flex-col gap-10`}>
					<div class='flex gap-10 items-center'>
						<h2 class='text-2xl font-semibold text-fore-60 flex-1'>{props.title}</h2>
						<IconButtonRender color={{ type: 'Fore', def: 60 }} name='mdi-close' size={30} event={props.cancelEvent} />
					</div>

					<p class='text-fore-40'>{props.description}</p>

					<div class='flex-1 min-h-0'>{props.body && <ComponentRender {...props.body} />}</div>

					{(props.cancelEvent || props.finishEvent) && (
						<div class='flex gap-20'>
							<div class='flex-1' />

							{props.cancelEvent && (
								<ButtonRender
									color={{ type: 'Fore', def: 30 }}
									full={false}
									label={props.cancelEventLabel || 'Cancel'}
									outline={false}
									size='Medium'
									event={props.cancelEvent}
								/>
							)}

							{props.cancelEvent && (
								<ButtonRender
									color={{ type: 'Primary', def: 100 }}
									full={false}
									label={props.finishEventLabel || 'Ok'}
									outline={false}
									size='Medium'
									event={props.finishEvent}
								/>
							)}
						</div>
					)}
				</div>
			</div>
		</div>
	)
}

const ModalContext = React.createContext<((modal: Modal) => void) | null>(null)

export interface ModalProviderProps {
	children: React.ReactNode
}

export function ModalProvider(props: ModalProviderProps) {
	const [modal, setModal] = React.useState<Modal | null>(null)

	return (
		<>
			<ModalContext.Provider value={(modal) => setModal(modal)}>{props.children}</ModalContext.Provider>

			{modal && <div class='fixed'></div>}
		</>
	)
}
