import { HTMLAttributes, ReactElement, MouseEvent } from "react";

import "./component.css";

interface ButtonProps extends Omit<HTMLAttributes<HTMLDivElement>, "innerHTML" | "innerText"> {
	children: string;
	type: "primary" | "secondary" | "warning" | "error";
	icon?: ReactElement;
	href?: string;
	disabled?: boolean;
}

export default function Button(props: ButtonProps): ReactElement {
	const {children, type, icon, href, onClick, className, disabled, ...rest} = props;

	function redirect(event: MouseEvent<HTMLDivElement>): void {
		if (disabled == true) // type safety.
			return;

		if (href != undefined) {
			location.assign(href);
		} else if (onClick != undefined) {
			onClick(event);
		}
	}

	return <div
		className={`button-${type} ${className}`}
		onClick={redirect}
		data-disabled={disabled ?? false}
		{...rest}
	>
		<span>{children}</span>
		{icon ?? ""}
	</div>;
}
