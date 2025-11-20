// Configuración del contrato
const CONTRACT_ID = "TU_CONTRACT_ID_AQUI"; // Actualizar después del despliegue
const NETWORK_PASSPHRASE = 'Test SDF Network ; September 2015';
const RPC_URL = 'https://soroban-testnet.stellar.org';

// Elementos del DOM
const connectWalletBtn = document.getElementById('connectWallet');
const walletInfo = document.getElementById('walletInfo');
const walletAddress = document.getElementById('walletAddress');
const registerForm = document.getElementById('registerForm');
const transferForm = document.getElementById('transferForm');
const signForm = document.getElementById('signForm');
const result = document.getElementById('result');
const resultContent = document.getElementById('resultContent');

// Variables globales
let connectedWallet = null;

// Conectar wallet
connectWalletBtn.addEventListener('click', async () => {
    try {
        const address = await window.freighterApi.getPublicKey();
        connectedWallet = address;
        walletAddress.textContent = address;
        walletInfo.classList.remove('hidden');
        connectWalletBtn.textContent = 'Wallet Conectada ✓';
        connectWalletBtn.disabled = true;
        showResult('Wallet conectada exitosamente!', 'success');
    } catch (error) {
        showResult(`Error conectando wallet: ${error.message}`, 'error');
    }
});

// Registrar recurso
registerForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    if (!connectedWallet) {
        showResult('Por favor conecta tu wallet primero', 'error');
        return;
    }

    const resourceData = {
        id: document.getElementById('resourceId').value,
        name: document.getElementById('resourceName').value,
        type: document.getElementById('resourceType').value,
        quantity: parseInt(document.getElementById('quantity').value),
        origin: document.getElementById('origin').value,
        carbonFootprint: parseInt(document.getElementById('carbonFootprint').value)
    };

    try {
        showResult('Registrando recurso...', 'info');
        
        // Aquí iría la llamada al contrato
        // Por ahora simulamos éxito
        setTimeout(() => {
            showResult(`Recurso ${resourceData.id} registrado exitosamente!`, 'success');
            registerForm.reset();
        }, 2000);
        
    } catch (error) {
        showResult(`Error registrando recurso: ${error.message}`, 'error');
    }
});

// Crear transferencia
transferForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    if (!connectedWallet) {
        showResult('Por favor conecta tu wallet primero', 'error');
        return;
    }

    const transferData = {
        resourceId: document.getElementById('transferResourceId').value,
        to: document.getElementById('toAddress').value,
        quantity: parseInt(document.getElementById('transferQuantity').value)
    };

    try {
        showResult('Creando transferencia...', 'info');
        
        // Simular creación de transferencia
        setTimeout(() => {
            showResult(`Transferencia creada con ID: ${Math.floor(Math.random() * 1000)}`, 'success');
            transferForm.reset();
        }, 2000);
        
    } catch (error) {
        showResult(`Error creando transferencia: ${error.message}`, 'error');
    }
});

// Firmar transacción
signForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    if (!connectedWallet) {
        showResult('Por favor conecta tu wallet primero', 'error');
        return;
    }

    const txId = parseInt(document.getElementById('txId').value);

    try {
        showResult(`Firmando transacción ${txId}...`, 'info');
        
        // Simular firma
        setTimeout(() => {
            showResult(`Transacción ${txId} firmada exitosamente!`, 'success');
            signForm.reset();
        }, 2000);
        
    } catch (error) {
        showResult(`Error firmando transacción: ${error.message}`, 'error');
    }
});

// Función para mostrar resultados
function showResult(message, type) {
    resultContent.textContent = message;
    result.className = `result ${type}`;
    result.classList.remove('hidden');
    
    // Scroll to result
    result.scrollIntoView({ behavior: 'smooth', block: 'end' });
}
