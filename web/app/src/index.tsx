/* @refresh reload */
import { render } from "solid-js/web";

import "@picocss/pico/css/pico.classless.min.css";
import "@matsunoki/ui/lib/style.css";
import { App } from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
