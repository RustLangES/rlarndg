import { ReactElement } from "react";
import { LogIn } from "lucide-react";

import TopBar from "../../components/top-bar/component";
import RandomText from "../../components/random-text/component";
import Button from "../../components/button/component";

import "./page.css";

export default function Landing(): ReactElement {
	return <>
		<TopBar links={[
			{text: "Documentation", href: "/docs"},
			{text: "Pricing", href: "/pricing"}
		]}/>
		<div className="landing-middle-wrapper">
			<div className="landing-title-wrapper">
				<div>
					<h1>
						RlARndG, An actual
						<br/>
						(not pseudo) random
						<br/>
						number generator.
					</h1>
					<div>
						<Button type="primary" href="/pricing">Get started</Button>
						<Button type="secondary" href="/docs">Documentation</Button>
					</div>
				</div>
				<div>
					<RandomText format="hex" />
					<RandomText format="decimal" />
					<RandomText format="binary" />
					<span className="landing-title-disclaimer">
						*The displayed numbers are pseudorandom.
					</span>
				</div>
			</div>
			<div className="landing-how-it-works">
				<h1>How does this work?</h1>
				<div>
					<div className="image-display">
						<img src="/times-square.jpg" alt="crowded place" />
						<p>
							We have multiple cameras in undisclossed crowded<br />
							locations where everything is different everytime.<br />
							Normally one can't predict what +5000 people are<br />
							doing at the same time.
						</p>
					</div>
					<div className="image-display">
						<img src="/rust-snippet.png" alt="rust snippet" />
						<p>
							Using the Rust, A screenshot from one of<br />
							the cameras choosen by convenience is taken<br />
							and a number is generated based on the image<br />
							bits.
						</p>
					</div>
					<div className="image-display">
						<img src="/http-server.png" alt="http server" />
						<p>
							Then, using actix web a web server that communicates<br />
							the random number generator is deployed, it converts<br />
							the response to json and makes an actual useable API<br />
							for other developers.
						</p>
					</div>
				</div>
			</div>
			<div className="landing-any-questions">
				<div>
					<h1>Have any questions?</h1>
					<p>
						You can join our wholesome rust community<br />
						even tho it's an spanish community, there are<br />
						english channels, and we offer support for this service<br />
						along with any coding questions you may have.
					</p>
					<Button type="primary" icon={<LogIn />}>Join the discord!</Button>
				</div>
				<div>
					<iframe src="https://discord.com/widget?id=778674594856960012&theme=dark" />
				</div>
			</div>
			<footer>
				<p>No Copyright RustLangEs, Licensed as CC0.</p>
			</footer>
		</div>
	</>;
}
