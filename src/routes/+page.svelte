<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  // Usaremos el comando 'get_current_user' mock que definimos en Rust
  let message: string = 'Iniciando conexión con el backend...';

  onMount(async () => {
    try {
      // Llamamos al comando de Rust
      const user = await invoke('get_current_user');
      message = `¡Conexión Exitosa! Usuario retornado desde Rust: ${user}`;

    } catch (error) {
      console.error(error);
      message = `Error de Comunicación con Rust: ${error}`;
    }
  });

</script>

<main class="container">
  <h1>Arquitecto App - Almacén de Víveres FAA</h1>
  <p>Estado del Backend Rust/Tauri:</p>
  <p><strong>{message}</strong></p>
  <p>Si ve un mensaje de usuario, el puente Rust 🦀 / Svelte ✨ está funcional.</p>
</main>

<style>
  /* Puedes limpiar o mantener los estilos por defecto aquí */
</style>