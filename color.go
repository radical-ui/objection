// Adapted from https://github.com/g4s8/hexcolor/blob/ba64ac840df3956eee30add50ff2f530780c4d0f/hexcolor.go

package objection

import "errors"

type Color struct {
	Red   byte `json:"red"`
	Green byte `json:"green"`
	Blue  byte `json:"blue"`
	Alpha byte `json:"alpha"`
}

// ErrInvalidFormat indicates that format of hex color
// string is invalid.
var ErrInvalidFormat = errors.New("invalid hex color format")

// Parse hex string to an objection.Color. It takes #RGB, #RRGGBB, #ARGB, #AARRGGBB strings as input
func ParseColor(hex string) (c Color, err error) {
	// got this parsing code from here https://stackoverflow.com/a/54200713/1723695

	c.Alpha = 0xff

	if hex[0] != '#' {
		return c, ErrInvalidFormat
	}

	hexToByte := func(b byte) byte {
		switch {
		case b >= '0' && b <= '9':
			return b - '0'
		case b >= 'a' && b <= 'f':
			return 10 + b - 'a'
		case b >= 'A' && b <= 'F':
			return 10 + b - 'A'
		}
		err = ErrInvalidFormat
		return 0
	}

	switch len(hex) {
	case 9:
		c.Red = (hexToByte(hex[1]) << 4) + hexToByte(hex[2])
		c.Green = (hexToByte(hex[3]) << 4) + hexToByte(hex[4])
		c.Blue = (hexToByte(hex[5]) << 4) + hexToByte(hex[6])
		c.Alpha = (hexToByte(hex[7]) << 4) + hexToByte(hex[8])
	case 7:
		c.Red = (hexToByte(hex[1]) << 4) + hexToByte(hex[2])
		c.Green = (hexToByte(hex[3]) << 4) + hexToByte(hex[4])
		c.Blue = (hexToByte(hex[5]) << 4) + hexToByte(hex[6])
	case 5:
		c.Red = hexToByte(hex[1]) * 17
		c.Green = hexToByte(hex[2]) * 17
		c.Blue = hexToByte(hex[3]) * 17
		c.Alpha = hexToByte(hex[4]) * 17
	case 4:
		c.Red = hexToByte(hex[1]) * 17
		c.Green = hexToByte(hex[2]) * 17
		c.Blue = hexToByte(hex[3]) * 17
	default:
		err = ErrInvalidFormat
	}
	return
}
