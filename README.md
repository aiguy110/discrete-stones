# Description
A friend recently brought up the question, "If you're given a bag of stones to weigh, 
and digital scale with a precision of 1 gram (floor of actual weight is displayed),
and you then learn weighing each stone individually that no stone wieghs more than 1 gram
(scale shows weight of each stone is 0)... what can you learn about the weights of each 
stone by weighing combinations of stones and noting the results?

The friend is more interested in a an analytical solution that, for each stone, provides
a definite interval which that stone's weight is guaranteed to lie within. The code in this 
repo impliments a lazier approach loosely inspired by Stocastic Gradient Decent:

For each stone, start with a wild guess about the stone's weight. Then take random samples of
multiple stones and compare their "hypothetical" combined weight to the actual weight reported 
by the scale. If there is an error between these values, adjust the hypothetical wieght of 
each stone in the sample to that the combined hypothetical weight is made closer to the real
weight.
