<script>
	import { user } from "./js/store.js";
	import { getSession } from "./js/auth.js";
	import LogIn from "./pages/Login.svelte";
	import LogOut from "./pages/Logout.svelte";
	import Secure from "./pages/Secure.svelte";
	import Expenses from "./pages/Expenses.svelte";
	import Apicheck from "./pages/Apicheck.svelte";
	import { onMount } from "svelte";

	let menu;
	$: loggedin = $user !== "";

	// check if logged in
	onMount(getSession);

	const set_menu_items = (loggedin) => {
		if (loggedin) {
			return [
				{ label: "About", id: 1 },
				{ label: "Secure", id: 3 },
				{ label: "API Check", id: 5 },
				{ label: "Logout", id: 4 },
			];
		} else {
			return [
				{ label: "About", id: 1 },
				{ label: "API Check", id: 5 },
				{ label: "Login", id: 2 },
			];
		}
	};
</script>

<Expenses />
