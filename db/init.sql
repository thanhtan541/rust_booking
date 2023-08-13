CREATE TABLE booking (
    id SERIAL PRIMARY KEY,
    start_date character varying(255) NOT NULL,
    end_date character varying(255) NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO booking (id, start_date, end_date) VALUES 
(1, '01/02/2022', '02/03/2022'), 
(2, '02/03/2022', '04/05/2022'), 
(3, '04/05/2022', '06/07/2022'), 
(4, '06/07/2022', '08/09/2022'), 

