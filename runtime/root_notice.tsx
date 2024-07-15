import { React } from './deps.ts'
import { IconRender } from './icon.tsx'
import { useNotices } from './notices.tsx'
import { Notice } from './types.ts'

interface NoticeRenderProps {
	notice: Notice
	dismiss(): void
}

function NoticeRender(props: NoticeRenderProps) {
	return (
		<div class='bg-base rounded overflow-hidden shadow-lg fixed right-20 bottom-20'>
			<div
				class={`
					p-20 flex gap-10 items-center
					${props.notice.style === 'Success' ? 'text-success bg-success-10' : 'text-danger bg-danger-10'}
				`}
			>
				<IconRender
					name={props.notice.style === 'Success' ? 'mdi-check-bold' : 'mdi-close-thick'}
					color={{ type: props.notice.style === 'Success' ? 'Success' : 'Danger', opacity: 100 }}
					size={20}
				/>
				<div>{props.notice.message}</div>
			</div>
		</div>
	)
}

export function RootNotice() {
	const [notice, clear] = useNotices()

	React.useEffect(() => {
		if (!notice) return void null

		const timeout = setTimeout(() => clear(), 5000)

		return () => clearTimeout(timeout)
	}, [notice])

	return notice ? <NoticeRender dismiss={clear} notice={notice} /> : <></>
}
