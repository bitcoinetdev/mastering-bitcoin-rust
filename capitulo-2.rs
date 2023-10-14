// Capítulo 2: Como funciona Bitcoin

/*
Este capítulo explica los componentes técnicos clave que conforman 
la red y el sistema Bitcoin:

- Direcciones Bitcoin
- Transacciones 
- Bloques
- La cadena de bloques
- Minería 
- El pool de minería
- El proceso de consenso
*/

// Direcciones Bitcoin 

/*
Las direcciones Bitcoin son identificadores que representan destinos 
para pagos. Permiten recibir bitcoins.

Una dirección es un hash de 160-bits de la clave pública del destinatario.
*/

// Transacciones Bitcoin

/*
Las transacciones son mensajes digitales firmados que representan
transferencias de bitcoins entre direcciones. Incluyen:

- Una o más entradas con referencias a las monedas a gastar
- Una o más salidas con nuevos destinos y cantidades
- Firma digital para autorizar el gasto

Cada entrada debe ser válida para que la transacción sea aceptada.
*/

// Estructura de las transacciones

/* 
Cada transacción está identificada por su hash de 256 bits.

Contiene metadata como versión, tiempo de creación, etc.

Las entradas referencian transacciones previas por hash y número de salida.

Las salidas contienen scripts para determinar el nuevo propietario. 

Finalmente, la transacción tiene una firma digital del remitente.
*/

// Bloques de la cadena de bloques

/*
Los bloques son conjuntos de transacciones con un header que contiene metadata.

Cada bloque hace referencia al bloque previo por su hash, vinculándolos en la cadena.

El header contiene información como:

- Versión del software 
- Timestamp
- Nonce y dificultad objetivo
- Raíz del merkle tree de transacciones
- Hash del header del bloque previo

Los bloques inicialmente tenían límite de 1MB pero luego aumentó. 
*/

// Minería de bloques

/*
Los mineros compiten para crear nuevos bloques siguiendo un consenso:

1. Recolectan transacciones para incluir
2. Verifican validez de transacciones
3. Resuelven puzzle criptográfico del bloque 
4. Propagan bloque a la red

El primer minero en resolver el puzzle gana la recompensa del bloque.
*/

// Minería en pool

/*
La minería en pool permite que mineros trabajen conjunto y compartan 
recompensas. El pool opera un nodo completo y distribuye trabajo.

Los beneficios incluyen:

- Pagos más estables y frecuentes para mineros
- Mayor poder computacional en conjunto
- Participación proporcional a trabajo aportado
- Menor varianza en los ingresos

Los pools cobran una pequeña comisión por administrar el grupo.
*/

// Proceso de consenso 

/*
El protocolo Bitcoin utiliza un proceso de consenso para garantizar que 
la red sincronice una única historia de transacciones.

Los nodos validan nuevos bloques e ignores ramas inválidas.
*/

// Fin de la traducción del Capítulo 2
