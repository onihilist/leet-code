/*
    Runtime : 597ms    (Beats 65.27%)
    Memory  : N/A
*/

SELECT player_id, MIN(event_date) AS first_login
FROM Activity
GROUP BY player_id
ORDER BY first_login ASC;