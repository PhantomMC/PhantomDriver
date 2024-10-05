SELECT disconnect_msg FROM (
        (SELECT address_name, response_id FROM AddressNames WHERE address_name = $1)
        INNER JOIN Responses ON response_id = id;
    );