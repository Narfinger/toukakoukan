<script>
	import { user } from "./js/store.js";
	import { getSession } from "./js/auth.js";
	import LogIn from "./pages/Login.svelte";
	import LogOut from "./pages/Logout.svelte";
	import Secure from "./pages/Secure.svelte";
	import Expenses from "./pages/Expenses.svelte";
	import Apicheck from "./pages/Apicheck.svelte";
	import Login from "./pages/Login.svelte";
	import { onMount } from "svelte";

	let menu;
	$: loggedin = $user !== "";

	// check if logged in
	onMount(getSession);
</script>

<Login />
