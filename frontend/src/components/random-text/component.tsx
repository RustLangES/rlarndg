import { ReactElement, useEffect, useRef, useState } from "react";

import "./component.css";

interface RandomTextProps {
	format: "hex" | "decimal" | "binary";
}

const defaultText = {
	"hex": "FFFFFF",
	"decimal": "123456",
	"binary": "010101"
};

const textPrefix = {
	"hex": "0x",
	"decimal": "00",
	"binary": "0b"
}

export default function RandomText({format}: RandomTextProps): ReactElement {
	const [text, setText] = useState(defaultText[format]);
	const pending = useRef<string[]>([]);

	useEffect(() => {
		function getRandomHex(): string {
			let hex = "";

			for (let i = 0; i < 6; i++)
				hex += Math.floor(Math.random() * 16)
					.toString(16);

			return hex;
		}

		function getRandomBinary(): string {
			let binary = "";

			for (let i = 0; i < 6; i++)
				binary += Math.floor(Math.random() * 2);

			return binary
		}

		function getRandomDecimal(): string {
			return Math.floor(100000 + Math.random() * 900000)
				.toString();
		}

		let interval = setInterval(() => {
			if (pending.current.length == 0) {
				pending.current =
					(format == "hex"
						? getRandomHex()
						: format == "binary"
							? getRandomBinary()
							: getRandomDecimal())
						.toUpperCase()
						.split("");
			}

			let nextChar = pending
				.current
				.shift();

			setText((prev) => {
				const position = prev.length - pending.current.length - 1;

				return (
					prev.slice(0, position) +
					nextChar +
					prev.slice(position + 1)
				)
			});
		}, 500);

		return () => {
			clearInterval(interval);
		};
	}, [text]);

	return <span>
		{textPrefix[format]}{text}
	</span>;
}
