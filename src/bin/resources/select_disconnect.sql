SELECT disconnect_msg FROM (
        (SELECT address_name, response_id FROM address_names WHERE address_name = $1)
        INNER JOIN responses ON response_id = id
    );