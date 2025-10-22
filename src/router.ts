// src/router.ts (CONTENIDO AUTO-CONTENIDO)
import InicioSesion from './pages/InicioSesion.svelte';
import Layout from './components/Layout.svelte';
import Contratos from './pages/Contratos.svelte';
import Salidas from './pages/Salidas.svelte';
import RecursosFinancieros from './pages/RecursosFinancieros.svelte';

// Definición de las rutas de la aplicación:
// La clave es el path, el valor es el componente Svelte.
const routes = {
    // La ruta raíz ('/') redirigirá al componente de inicio de sesión.
    '/': InicioSesion,

    // El Layout contendrá todas las páginas que requieren autenticación/navegación.
    '/app': Layout, 
    '/app/contratos': Contratos,
    '/app/salidas': Salidas,
    '/app/finanzas': RecursosFinancieros,

    // Catch-all para errores 404
    '*': InicioSesion, 
};

export default routes;