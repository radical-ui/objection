import { ActionKey, React, registerActionListener } from './deps.ts'

export function useAction<T>(key: ActionKey<T> | null, listener: (data: T) => void) {
	React.useEffect(() => {
		if (!key) return

		return registerActionListener(key, listener)
	}, [key])
}
