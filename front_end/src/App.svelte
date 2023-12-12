<script>
	import { user } from "./js/store.js";
	import { getSession } from "./js/auth.js";
	import LogIn from "./pages/Login.svelte";
	import LogOut from "./pages/Logout.svelte";
	import Secure from "./pages/Secure.svelte";
	import Expenses from "./pages/Expenses.svelte";
	import Apicheck from "./pages/Apicheck.svelte";
	import Login from "./pages/Login.svelte";
	import AddExpense from "./pages/AddExpense.svelte";
	import { onMount } from "svelte";

	const routingMap = {
		"": Login,
		"#expenses": Expenses,
		"#login": Login,
		"#addexpense": AddExpense,
	};
	let page = Login;
	function routeChange() {
		page = routingMap[location.hash] || Login;
	}
	$: loggedin = $user !== "";

	// check if logged in
	onMount(getSession);
</script>

<svelte:window on:hashchange={routeChange} />

<svelte:component this={page} />
