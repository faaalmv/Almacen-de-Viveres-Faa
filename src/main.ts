// src/main.ts (CONTENIDO AUTO-CONTENIDO)
import App from './App.svelte';
import routes from './router';
import Router from 'svelte-spa-router'; // Importamos el router instalado

const app = new App({
  target: document.body,
  props: {
    // Redirigimos la aplicación al componente Router, pasándole las rutas
    component: Router, 
    props: {
      routes: routes,
    }
  },
});

export default app;