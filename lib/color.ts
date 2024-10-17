export class Color {
	red: number
	green: number
	blue: number

	constructor(red: number, green: number, blue: number) {
		this.red = red
		this.green = green
		this.blue = blue
	}

	static fromHex(text: string) {
		// reference: https://stackoverflow.com/a/5624139
		// Expand shorthand form (e.g. "03F") to full form (e.g. "0033FF")

		const shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i
		text = text.trim()
		text = text.replace(shorthandRegex, function (_, r, g, b) {
			return r + r + g + g + b + b
		})

		const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(text)
		if (!result) throw new Error(`failed to parse hex color: ${text}`)

		return new Color(
			parseInt(result[1], 16),
			parseInt(result[2], 16),
			parseInt(result[3], 16),
		)
	}

	toHex() {
		return '#' + (1 << 24 | this.red << 16 | this.green << 8 | this.blue).toString(16).slice(1)
	}

	toObject(): unknown {
		return { red: this.red, green: this.green, blue: this.blue }
	}
}
