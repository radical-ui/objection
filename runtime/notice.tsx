import { useAction } from './action.tsx'
import { Component, ComponentRender } from './component.tsx'
import { ActionKey, React } from './deps.ts'
import { IconRender } from './icon.tsx'
import { GlobalCss } from './utils.ts'

export interface Notice {
	message: string
	style: NoticeStyle
}

export type NoticeStyle = 'Error' | 'Success'

interface NoticeContext {
	addNotice(notice: Notice): void
}

const Context = React.createContext<NoticeContext | null>(null)

/** Returns a function to dispatch a notice */
export function useNoticeDispatch() {
	const context = React.useContext(Context)
	if (!context) return null

	return context.addNotice
}

/**
 * A notice manager for displaying notices on the screen
 *
 * @component
 */
export interface NoticeManager {
	add_notice_action: ActionKey<Notice>
	body: Component
}

export function NoticeManagerRender(props: NoticeManager) {
	const [notices, setNotices] = React.useState<Notice[]>([])
	const currentNotice = notices[0] ?? null

	const dismiss = () => {
		setNotices(notices.slice(1))
	}

	useAction(props.add_notice_action, (notice) => {
		setNotices((notices) => [...notices, notice])
	})

	const context = React.useMemo<NoticeContext>(() => ({
		addNotice(notice: Notice) {
			setNotices((notices) => [...notices, notice])
		},
	}), [])

	return (
		<div class='w-full h-full relative'>
			<Context.Provider value={context}>
				<ComponentRender {...props.body} />
			</Context.Provider>

			{currentNotice && <SingleNotice dismiss={dismiss} {...currentNotice} />}
		</div>
	)
}

const singleNoticeCss = new GlobalCss(`
	.single-notice {
		animation: singlie-notice-fade-in ease-out 0.3s;
	}

	@keyframes single-notice-fade-in {
		from {
			bottom: -100px;
		}
		to {
			bottom: 20px;
		}
	}
`)

function SingleNotice(props: Notice & { dismiss(): void }) {
	singleNoticeCss.present()

	return (
		<div class='bg-base rounded overflow-hidden shadow-lg fixed right-20 bottom-20'>
			<div
				class={`
					p-20 flex gap-10 items-center
					${props.style === 'Success' ? 'text-success bg-success-10' : 'text-danger bg-danger-10'}
				`}
			>
				<IconRender
					name={props.style === 'Success' ? 'mdi-check-bold' : 'mdi-close-thick'}
					color={{ type: props.style === 'Success' ? 'Success' : 'Danger', opacity: 100 }}
					size={20}
				/>
				<div>{props.message}</div>
			</div>
		</div>
	)
}
