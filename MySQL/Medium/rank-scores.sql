/*
    Runtime : 424ms    (Beats 61.17%)
    Memory  : N/A
*/

WITH RankedScores AS (
    SELECT 
        score,
        DENSE_RANK() OVER (ORDER BY score DESC) AS score_rank
    FROM 
        scores
)
SELECT 
    score,
    score_rank AS 'rank'
FROM 
    RankedScores
ORDER BY 
    score DESC;