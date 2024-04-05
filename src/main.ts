import App from "./App.svelte";
import { tauri_config } from "./tauri";

tauri_config();
const app = new App({
  target: document.getElementById("app") as HTMLElement,
});

export default app;
