<script lang="ts">
	import { user } from "./js/store";
	import { getSession } from "./js/auth";
	import MainView from "./pages/MainView.svelte";
	import Login from "./pages/Login.svelte";
	import AddExpense from "./pages/AddExpense.svelte";
	import AddGroup from "./pages/AddGroup.svelte";
	import CreateUser from "./pages/CreateUser.svelte";
	import { onMount } from "svelte";
	import Router from "svelte-spa-router";
	import NotFound from "./pages/NotFound.svelte";
	import EditExpense from "./pages/EditExpense.svelte";
	import { useRegisterSW } from "virtual:pwa-register/svelte";

	const intervalMS = 24 * 60 * 60 * 1000; //one day
	const updateServiceWorker = useRegisterSW({
		onRegistered(r) {
			r &&
				setInterval(() => {
					r.update();
				}, intervalMS);
		},
	});

	$: loggedin = $user !== "";

	const routes = {
		"/": Login,
		"/expenses": MainView,
		"/addexpense/:id": AddExpense,
		"/addgroup/": AddGroup,
		"/createuser/": CreateUser,
		"/edit/:id": EditExpense,
		"*": NotFound,
	};

	// check if logged in
	onMount(getSession);
</script>

<body>
	<Router {routes} />
</body>
