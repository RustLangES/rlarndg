import { HTMLAttributes, ReactElement } from "react";

import "./component.css";

interface ButtonProps extends Omit<HTMLAttributes<HTMLDivElement>, "innerHTML" | "innerText"> {
	children: string;
	type: "primary" | "secondary" | "warning" | "error";
	icon?: ReactElement;
}

export default function Button({children, type, icon, className, ...rest}: ButtonProps): ReactElement {
	return <div className={`button-${type} ${className}`} {...rest}>
		<span>{children}</span>
		{icon ?? ""}
	</div>;
}
