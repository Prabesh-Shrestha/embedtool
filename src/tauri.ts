import { appWindow, PhysicalSize } from "@tauri-apps/api/window";

export const tauri_config = async () => {
  await appWindow.setMinSize(new PhysicalSize(800, 500));
};
