<script lang="ts">
    import { push } from 'svelte-spa-router';
    import { currentUser } from '../stores';
    import { invoke } from '@tauri-apps/api/tauri';
    
    // El Layout recibe la ruta (hash) actual para saber qué componente hijo renderizar.
    export let location: string; 

    // Variable reactiva para el estado del usuario autenticado
    let user = $currentUser;
    
    // Suscribirse a los cambios del store
    $: user = $currentUser;
    
    // Función para navegar
    function navigateTo(path: string) {
        push(path);
    }

    // Función de logout
    async function handleLogout() {
        // En un futuro, aquí se llamaría a invoke('logout')
        currentUser.set(null); // Limpiamos el store
        push('/'); // Redirigimos a la página de login
    }

    // Si el usuario no está autenticado, redirigir al login
    if (!user) {
        push('/');
    }
</script>

{#if user}
    <div class="layout-container">
        <aside class="sidebar">
            <h3>Almacén FAA</h3>
            <div class="user-info">
                <p>Bienvenido, <strong>{user.name}</strong></p>
                <p class="role">Rol: {user.role}</p>
            </div>
            
            <nav class="nav-menu">
                <a 
                    href="#/app/contratos" 
                    on:click={() => navigateTo('/app/contratos')}
                    class:active={location.includes('/contratos')}
                >
                    Contratos
                </a>
                <a 
                    href="#/app/salidas" 
                    on:click={() => navigateTo('/app/salidas')}
                    class:active={location.includes('/salidas')}
                >
                    Salidas (Inventario)
                </a>
                <a 
                    href="#/app/finanzas" 
                    on:click={() => navigateTo('/app/finanzas')}
                    class:active={location.includes('/finanzas')}
                >
                    Recursos Financieros
                </a>
            </nav>
            
            <button class="logout-btn" on:click={handleLogout}>Cerrar Sesión</button>
        </aside>

        <main class="content">
            <slot></slot>
        </main>
    </div>
{:else}
    {/if}

<style>
    .layout-container {
        display: flex;
        height: 100vh;
        width: 100vw;
    }
    .sidebar {
        width: 250px;
        background-color: #333;
        color: white;
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }
    .user-info {
        border-bottom: 1px solid #555;
        padding-bottom: 15px;
        margin-bottom: 15px;
    }
    .role {
        font-size: 0.9em;
        color: #bbb;
    }
    .nav-menu {
        display: flex;
        flex-direction: column;
        flex-grow: 1; /* Permite que el menú crezca */
    }
    .nav-menu a {
        padding: 10px 0;
        text-decoration: none;
        color: white;
        border-radius: 4px;
        margin-bottom: 5px;
        transition: background-color 0.2s;
    }
    .nav-menu a:hover, .nav-menu a.active {
        background-color: #555;
        padding-left: 10px;
    }
    .content {
        flex-grow: 1;
        padding: 20px;
        overflow-y: auto;
    }
    .logout-btn {
        background-color: #dc3545;
        color: white;
        border: none;
        padding: 10px;
        cursor: pointer;
        border-radius: 4px;
    }
</style>