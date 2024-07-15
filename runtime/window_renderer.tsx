import { ProvideDispatch } from './action.tsx'
import { RawDispatchFn } from './action.tsx'
import { ComponentRender } from '../components/mod.tsx'
import { frontier, React } from './deps.ts'
import { ProvideNotices } from './notices.tsx'
import { ColorDefinition, ColorPalette, Component, Window } from './types.ts'
import { ProvideUpdateManager, UpdateManager } from './component_update.tsx'
import { Notice } from 'runtime'
import { RootNotice } from './root_notice.tsx'
import { ModalProvider } from '../components/modal/mod.tsx'

export class WindowRenderer {
	#internalUpdate: ((root: Component) => void) | null = null
	#internalAddNotice: ((notice: Notice) => void) | null = null
	#actionHandler: RawDispatchFn | null = null
	#updateManager = new UpdateManager()
	#didGiveTheme = false

	#toColor(def: ColorDefinition) {
		return [def.red, def.green, def.blue] as frontier.Color
	}

	#convertPalette(palette: ColorPalette): frontier.Palette {
		return {
			primary: this.#toColor(palette.primary),
			secondary: this.#toColor(palette.secondary),
			base: this.#toColor(palette.base),
			fore: this.#toColor(palette.fore),
			decorationFore: this.#toColor(palette.decoration_fore),
			danger: this.#toColor(palette.danger),
			warn: this.#toColor(palette.warn),
			success: this.#toColor(palette.success),
			notice: this.#toColor(palette.notice),
		}
	}

	setActionHandler(dispatch: RawDispatchFn) {
		this.#actionHandler = async (tree) => {
			try {
				return await dispatch(tree)
			} catch (error) {
				const message = error.message ?? error

				this.notice({ message, style: 'Error' })
			}
		}

		return this
	}

	render(window: Window, rootElementOverride: HTMLElement | null = null) {
		if (document.title !== window.title) document.title = window.title

		if (window.theme) {
			frontier.setupDynamicTheme({
				lightPalette: this.#convertPalette(window.theme.light_palette),
				darkPalette: this.#convertPalette(window.theme.dark_palette),
				options: {
					roundBase: window.theme.round_base,
					selectionMode: window.theme.selection_mode === 'OptIn' ? 'opt-in' : 'opt-out',
					windowScrolling: window.theme.window_scrolling,
					defaultFont: window.theme.default_font ?? undefined,
					fancyFont: window.theme.fancy_font ?? undefined,
				},
			})

			this.#didGiveTheme = true
		} else if (!this.#didGiveTheme) {
			console.error('No theme was received from server')
		}

		const Main = this.#Main.bind(this)

		if (this.#internalUpdate) this.#internalUpdate(window.root_component)
		else {
			const rootElement = rootElementOverride ?? document.getElementById('root')
			if (!rootElement) {
				throw new Error('Expected to find a #root element. Either define one, or provide the `rootElementOverride` argument')
			}

			React.render(<Main {...window.root_component} />, rootElement)
		}
	}

	notice(notice: Notice) {
		if (this.#internalAddNotice) this.#internalAddNotice(notice)
	}

	applyUpdate(id: number, data: unknown) {
		this.#updateManager.update(id, data)
	}

	#Main(initialComponent: Component) {
		const [component, setComponent] = React.useState(initialComponent)

		React.useEffect(() => {
			this.#internalUpdate = (root) => setComponent(root)
		}, [])

		return (
			<ModalProvider>
				<ProvideNotices
					setNoticeListener={(fn) => {
						console.log('register')
						this.#internalAddNotice = fn
					}}
				>
					<ProvideUpdateManager manger={this.#updateManager}>
						<ProvideDispatch
							dispatch={(tree) => {
								if (this.#actionHandler) return this.#actionHandler(tree)

								console.warn('No action handler set')
								return Promise.resolve()
							}}
						>
							<ComponentRender {...component} />
							<RootNotice />
						</ProvideDispatch>
					</ProvideUpdateManager>
				</ProvideNotices>
			</ModalProvider>
		)
	}
}
