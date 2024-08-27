<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { listen } from "@tauri-apps/api/event";

	let ports: string[] = [];
	let selectedPort = "";
	let serialData: string[] = [];

	onMount(() => {
		async function setup() {
			try {
				const availablePorts: string[] =
					await invoke("list_serial_ports");
				ports = availablePorts;
			} catch (error) {
				console.error("Error fetching serial ports:", error);
			}

			// Listen for the serial-data event from the backend
			const unlisten = await listen("serial-data", (event) => {
				serialData = [...serialData, event.payload as string];
			});

			return () => {
				unlisten();
			};
		}
		setup();
	});
	async function startReadingSerial() {
		if (selectedPort) {
			try {
				await invoke("start_serial_reading", {
					portName: selectedPort,
				});
			} catch (error) {
				console.error("Error starting serial reading:", error);
			}
		}
	}
</script>
