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

<main>
	<h1>Serial Port Reader</h1>

	<!-- Dropdown to select a serial port -->
	<select bind:value={selectedPort}>
		<option value="" disabled>Select a port</option>
		{#each ports as port}
			<option value={port}>{port}</option>
		{/each}
	</select>

	<button on:click={startReadingSerial} disabled={!selectedPort}>
		Start Reading
	</button>

	<h2>Live Serial Data:</h2>
	<div>
		{#each serialData as data}
			<p>{data}</p>
		{/each}
	</div>
</main>

<style>
	main {
		font-family: Arial, sans-serif;
		margin: 0 auto;
		padding: 1rem;
		max-width: 600px;
	}
</style>
