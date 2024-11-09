import { ReactElement } from "react";

import TopBar from "../../components/top-bar/component";

export default function Documentation(): ReactElement {
	return <>
		<TopBar links={[
			{text: "Home", href: "/"},
			{text: "Pricing", href: "/pricing"}
		]}/>
	</>;
}
