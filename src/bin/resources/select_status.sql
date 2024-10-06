SELECT version_name, protocol, hover_text, max_players, players_online, favicon, secure_chat, motd FROM (
        (SELECT address_name, response_id FROM address_names WHERE address_name = $1)
        INNER JOIN responses ON response_id = id
    );