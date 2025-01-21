/*
    Runtime : 550 - 1182ms    (Beats 52.24 - 5.35%)
    Memory  : N/A
*/

SELECT Email FROM Person
GROUP BY email
HAVING COUNT(email)>1;