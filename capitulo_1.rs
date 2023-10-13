// Capítulo 1: Introducción a Bitcoin

/*
Este capítulo proporciona una breve introducción a Bitcoin, comenzando 
con un resumen de los principales componentes de la red Bitcoin. Luego
explica brevemente cómo se realizan transacciones en la cadena de bloques, 
cómo se validan y confirman.
*/

// Secciones de la red Bitcoin

/*
La red Bitcoin se compone de nodos que validan y retransmiten transacciones 
y bloques. Hay varios tipos de nodos:

- Nodos de minería - validan transacciones y bloques y generan hashes
- Nodos completos - validan, almacenan blockchain completa y retransmiten
- Nodos livianos - almacenan, validan y retransmiten, pero no toda la cadena
*/

// Más secciones explicando composición de la red y flujo de transacciones

// Validación de transacciones

/*
Las transacciones se transmiten a la red como "transacciones sin confirmar".
Los nodos completos validan la transacción verificando:

- Que la sintaxis sea correcta
- Que la transacción no esté duplicada
- Que las entradas sumen los valores de salida
- Otras reglas de consenso

Si la transacción es válida, se agrega al pool de memoria de transacciones 
no confirmadas. De allí se agregará a un bloque.
*/

// Minería de bloques

/*
Los mineros compiten para calcular hashes con valor menor al objetivo. 
El primero en lograrlo mina un bloque válido a la cadena. 

El protocolo ajusta la dificultad para mantener 10min por bloque.
*/

// Continuará la traducción del Capítulo 1...
