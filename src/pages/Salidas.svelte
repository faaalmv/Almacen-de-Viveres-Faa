<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    
    // 1. Tipos de datos alineados con los structs de Rust (models/inventory.rs)
    interface OrderItem {
        id: string;
        description: string;
        cantPedida: number; // camelCase para el frontend
        cantSurtida: number;
    }

    interface Folio {
        number: string;
    }

    // 2. Estado local
    let service: string = 'Dirección General';
    let errorMessage: string | null = null;
    let successFolio: string | null = null;
    let isLoading: boolean = false;
    
    // Estado inicial de los ítems a enviar
    let items: OrderItem[] = [
        { id: '101', description: 'Arroz Grano Largo', cantPedida: 500, cantSurtida: 500 },
        { id: '102', description: 'Frijol Negro', cantPedida: 300, cantSurtida: 0 },
        { id: '103', description: 'Azúcar Estándar', cantPedida: 150, cantSurtida: 200 } // Error de validación intencional
    ];

    // 3. Función de manejo de envío
    async function handleSubmit() {
        errorMessage = null;
        successFolio = null;
        isLoading = true;

        // Limpieza de datos (ej. asegurar que cantSurtida es un número)
        const cleanedItems = items.map(item => ({
            ...item,
            cantSurtida: Number(item.cantSurtida) // Asegura el tipo numérico para Rust
        }));

        try {
            // Invoca el comando de Rust (IPC)
            const result: Folio = await invoke('process_and_save_salida', {
                items: cleanedItems,
                service: service
            });

            // Éxito: Muestra el folio
            successFolio = result.number;

        } catch (error: any) {
            // Fracaso: Muestra el error estructurado de Rust (AppError)
            console.error('Error del Backend:', error);
            
            // La validación en Rust devolverá una estructura JSON.
            errorMessage = error.message || JSON.stringify(error);

        } finally {
            isLoading = false;
        }
    }
    
    // Función auxiliar para restablecer el formulario
    function resetForm() {
        successFolio = null;
        errorMessage = null;
    }
</script>

<div class="salidas-container">
    <h1>Salidas de Almacén</h1>
    <button on:click={resetForm} class="reset-btn">Nuevo Pedido</button>

    {#if successFolio}
        <div class="message success">
            ✅ Salida Procesada con Éxito. Folio Generado: <strong>{successFolio}</strong>.
            <p>El backend de Rust completó la transacción.</p>
        </div>
    {:else if errorMessage}
        <div class="message error">
            ❌ Error de Procesamiento:
            <p>{errorMessage}</p>
        </div>
    {/if}

    <form on:submit|preventDefault={handleSubmit} class="form-grid">
        <label>
            Servicio Solicitante:
            <input type="text" bind:value={service} required disabled={isLoading}>
        </label>
        
        <h2>Ítems del Pedido</h2>
        <table class="items-table">
            <thead>
                <tr>
                    <th>Descripción</th>
                    <th>Cantidad Pedida</th>
                    <th>Cantidad Surtida</th>
                </tr>
            </thead>
            <tbody>
                {#each items as item (item.id)}
                    <tr>
                        <td>{item.description}</td>
                        <td>{item.cantPedida}</td>
                        <td>
                            <input 
                                type="number" 
                                bind:value={item.cantSurtida} 
                                min="0" 
                                required 
                                disabled={isLoading}
                                class:exceeded={item.cantSurtida > item.cantPedida}
                            >
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>

        <button type="submit" disabled={isLoading} class="submit-btn">
            {isLoading ? 'Guardando...' : 'Procesar y Guardar Salida (Rust)'}
        </button>
    </form>
</div>

<style>
    .salidas-container {
        max-width: 900px;
        margin: 0 auto;
        padding: 20px;
    }
    .reset-btn {
        float: right;
        background-color: #ffc107;
        color: #333;
        border: none;
        padding: 10px 15px;
        cursor: pointer;
        border-radius: 4px;
        margin-top: 10px;
    }
    .form-grid {
        display: flex;
        flex-direction: column;
        gap: 20px;
        margin-top: 50px;
    }
    .items-table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 10px;
    }
    .items-table th, .items-table td {
        border: 1px solid #ddd;
        padding: 10px;
        text-align: left;
    }
    .items-table th {
        background-color: #f2f2f2;
    }
    .items-table input[type="number"] {
        width: 80px;
        padding: 5px;
        border: 1px solid #ccc;
    }
    .items-table input.exceeded {
        border-color: red;
        background-color: #ffeaea;
    }
    .submit-btn {
        background-color: #28a745;
        color: white;
        padding: 10px 15px;
        border: none;
        cursor: pointer;
        border-radius: 4px;
        margin-top: 10px;
    }
    .message {
        padding: 15px;
        border-radius: 4px;
        margin-bottom: 20px;
    }
    .message.success {
        background-color: #d4edda;
        color: #155724;
        border: 1px solid #c3e6cb;
    }
    .message.error {
        background-color: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }
</style>