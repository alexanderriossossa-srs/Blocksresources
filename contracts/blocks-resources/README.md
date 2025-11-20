# 📦 Block's Resources - Smart Contract

## 📁 Estructura del contrato

Este contrato inteligente está dividido en módulos para mantener el código limpio y escalable:

### `lib.rs`
- Punto de entrada del contrato
- Expone los métodos públicos del contrato
- Coordina entre los módulos `multisig` y `resource`

### `multisig.rs`
- Implementa la lógica de multifirma (2/3)
- Gestiona transacciones pendientes de aprobación
- Verifica autorizaciones y ejecuta transacciones cuando se alcanzan las firmas requeridas

### `resource.rs`
- Gestiona el registro y transferencia de recursos ambientales
- Cada recurso tiene: ID, nombre, tipo, cantidad, origen, huella de carbono, propietario y timestamp
- Permite la trazabilidad completa de recursos

## 🔐 Seguridad

- Solo los propietarios pueden transferir sus recursos
- Las transacciones requieren al menos 2 firmas de un grupo de 3 autorizados
- Las firmas se verifican individualmente antes de ejecutar
- Las transacciones ejecutadas no pueden modificarse

## 🧪 Testing

```bash
# Compilar
soroban contract build

# Ejecutar tests
cargo test
