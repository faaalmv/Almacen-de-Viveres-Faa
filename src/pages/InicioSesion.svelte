<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { currentUser, type User } from '../stores';
    import { push } from 'svelte-spa-router';

    // Estado reactivo (reemplazo de useState de React) 
    let userId: string = '';
    let password: string = '';
    let errorMessage: string | null = null;
    let isLoading: boolean = false;

    // Función que se ejecuta al enviar el formulario (reemplazo de onSubmit)
    async function handleSubmit() {
        errorMessage = null;
        isLoading = true;

        try {
            // Llama al comando de Rust con 'invoke' 
            const token = await invoke('login', { userId, password }); 
            
            // Simulación de los datos del usuario para el store (realmente vendría de Rust)
            const userData: User = {
                id: userId,
                name: `Usuario ${userId}`,
                role: 'Administrador', // Asignamos un rol por defecto para la prueba.
            };

            // 1. Actualiza el estado global de la UI 
            currentUser.set(userData);

            // 2. Redirige al layout principal 
            push('/app');

        } catch (error: any) {
            // Manejo de errores devueltos por Rust (AppError serializado) [cite: 54, 55]
            errorMessage = `Error de Login: ${error.message || JSON.stringify(error)}`;
            console.error('Login error:', error);
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="login-container">
    <h2>Inicio de Sesión - Almacén FAA</h2>
    <form on:submit|preventDefault={handleSubmit}>
        <input 
            type="text" 
            placeholder="ID de Usuario" 
            bind:value={userId} 
            disabled={isLoading} 
            required
        >
        <input 
            type="password" 
            placeholder="Contraseña" 
            bind:value={password} 
            disabled={isLoading} 
            required
        >
        
        {#if errorMessage}
            <p class="error-message">{errorMessage}</p>
        {/if}

        <button type="submit" disabled={isLoading}>
            {isLoading ? 'Autenticando...' : 'Entrar'}
        </button>
    </form>
</div>

<style>
    /* Estilos básicos para centrar el formulario */
    .login-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        width: 100vw;
        background: #f0f0f0;
    }
    form {
        display: flex;
        flex-direction: column;
        gap: 15px;
        padding: 40px;
        background: white;
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }
    input, button {
        padding: 10px;
        border-radius: 4px;
        border: 1px solid #ccc;
    }
    button {
        background-color: #007bff;
        color: white;
        cursor: pointer;
    }
    .error-message {
        color: red;
        text-align: center;
    }
</style>