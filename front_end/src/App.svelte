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
	import {
		route_addexpense,
		route_expense,
		route_login,
	} from "./js/consts.js";

	const routingMap = {
		"": Expenses,
		route_expense: Expenses,
		route_login: Login,
		route_addexpense: AddExpense,
	};
	let page = Login;
	function routeChange() {
		console.log(location.hash);
		page = routingMap[location.hash] || Login;
	}
	$: loggedin = $user !== "";

	// check if logged in
	onMount(getSession);
</script>

<svelte:window on:hashchange={routeChange} />

<svelte:component this={page} />
