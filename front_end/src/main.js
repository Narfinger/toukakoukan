import './app.css'
import App from './App.svelte'

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

const app = new App({
  target: document.getElementById('app')
})

export default app
