SELECT version_name, protocol, hover_text, max_players, players_online, favicon, secure_chat, motd FROM (
        (SELECT address_name, response_id FROM AddressNames WHERE address_name = $1)
        INNER JOIN Responses ON response_id = id;
    );