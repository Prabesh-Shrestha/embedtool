<script lang="ts">
	import Dashboard from "./lib/Dashboard.svelte";
	import Package from "./lib/Package.svelte";
	import Profile from "./lib/Profile.svelte";
	import Settings from "./lib/Settings.svelte";
	import Footer from "./lib/footer.svelte";
	type Page = "Dashboard" | "Package" | "Settings" | "Profile";
	let page: Page = "Dashboard";

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
	<nav>
		<ul>
			<li>
				<a
					href="#dashboard"
					on:click={() => {
						page = "Dashboard";
					}}>Dashboard</a
				>
			</li>
			<li>
				<a
					href="#package"
					on:click={() => {
						page = "Package";
					}}>Package Manager</a
				>
			</li>
			<li>
				<a
					href="#settings"
					on:click={() => {
						page = "Settings";
					}}>Settings</a
				>
			</li>
			<li>
				<a
					href="#profile"
					on:click={() => {
						page = "Profile";
					}}>Profile</a
				>
			</li>
		</ul>
	</nav>

	<div>
		{#if page == "Dashboard"}
			<Dashboard></Dashboard>
		{:else if page == "Package"}
			<Package></Package>
		{:else if page == "Profile"}
			<Profile></Profile>
		{:else if page == "Settings"}
			<Settings></Settings>
		{:else}
			<div>
				Something Went wrong here <br />
				go homepage
			</div>
		{/if}
	</div>
	<Footer></Footer>
</main>

<style>
	* {
		color: rgb(222, 218, 207);
		font-size: 20px;
	}
	li a {
		text-decoration: none;
	}
	nav ul {
		list-style: none;
		display: flex;
		justify-content: space-between;
		padding-right: 50px;
	}
</style>
