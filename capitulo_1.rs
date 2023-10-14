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

// Minado de un nuevo bloque

/*
Cuando se mina un nuevo bloque, éste se transmite a la red. Los nodos
completos validan el bloque:

- El hash previo debe ser correcto 
- Debe cumplir la dificultad objetivo
- Las transacciones deben ser válidas

Si el bloque es válido, se agrega a la cadena y se avanza el puntero.
Los nodos mantienen solo la cadena más larga.
*/

// Confirmaciones

/*
Las transacciones se consideran confirmadas cuando están suficientemente
enterradas en la blockchain. Por convención se esperan 6 confirmaciones.

Cada nueva confirmación reduce exponencialmente el riesgo de reversión.
*/

// Bloques huérfanos

/*
Cuando dos mineros generan un bloque al mismo tiempo, se crea un fork 
temporal en la cadena. Esto se resuelve cuando se mina el siguiente bloque.

El bloque descartado se conoce como "huérfano". Las transacciones 
en huérfanos vuelven al pool para ser minadas en otro bloque.
*/

// Incentivos financieros 

/*
Los mineros reciben una recompensa de nuevos bitcoins por cada bloque 
minado, empezando con 50 BTC en 2009 y reduciéndose a la mitad cada 
210.000 bloques.

También reciben las comisiones de transacción de ese bloque. Esto 
representa los incentivos financieros para mantener la red.
*/

// Fin de la traducción
