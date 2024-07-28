# Build and Install

To build and install the project, run the following command:
```bash
sudo cargo install --path <root_of_project>
```
This will generate and populate the systemd unit file in `/etc/systemd/system/`.

# Start and Enable the Service

To start service, run:
```bash
sudo systemctl start air_alert.service
```

To ensure the service starts automatically after a reboot or if the application crashes, run:
```bash
sudo systemctl enable air_alert.service
```

# Configuration

All configurations located at `samples/config.json`

After any change, run:
```bash
sudo cargo install --path <root_of_project>
sudo systemctl reload-or-restart air_alert.service
```

## Api Key

You need to obtain an API key from the official air alert API developer: [Ukraine Alarm API](https://api.ukrainealarm.com/).

Once you have the API key, update the `apiKey` field in `samples/config.json`.

## Observing region

To set the region you want to monitor, update the regionId field in `samples/config.json` with your actual region ID.

You can get all region IDs by running:
```bash
curl -X 'GET' \
  'https://api.ukrainealarm.com/api/v3/regions' \
  -H 'accept: application/json' \
  -H 'Authorization: <your_api_key>'
```
Remember to replace `<your_api_key>` with your actual API key.

If the command above does not work, you can find the command by visiting the [Ukraine Alarm API Swagger documentation](https://api.ukrainealarm.com/swagger/index.html).

## Sound samples

To change the audio that announces the start and end of an air alert, replace the `start_air_alert.mp3` and `end_air_alert.mp3` files in the `rsc` directory.

<hr>
<a href="https://stand-with-ukraine.pp.ua">
  <img src="https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg">
</a>
