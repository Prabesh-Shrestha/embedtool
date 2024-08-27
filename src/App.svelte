<script lang="ts">
	import Dashboard from "./lib/Dashboard.svelte";
	import Package from "./lib/Package.svelte";
  import Serial from "./lib/Serial.svelte"
	import Profile from "./lib/Profile.svelte";
	import Settings from "./lib/Settings.svelte";
	import Footer from "./lib/footer.svelte";
	type Page = "Dashboard" | "Serial" | "Package" | "Settings" | "Profile";
	let page: Page = "Dashboard";

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
					href="#serial"
					on:click={() => {
						page = "Serial";
					}}>Serial</a
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
		{:else if page == "Serial"}
			<Serial></Serial>
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
