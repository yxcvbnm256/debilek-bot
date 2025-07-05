# Debilek bot

Discord Bot, which greets users upon joining voice channels, provides a text-to-speech feature and plays simple audio clips, defined in the assets folder.

## Features:
- Greets users upon joining the voice channel, either with a user-specific greeting or generic fallback greeting. Leaves the voice channel if no user left in the voice channel.
- Plays Text-to-speech with different voices and languages
- Playing audio clips—the commands are automatically generated from assets (and subfolders) from the assets folder.

## Setup Guide:
### Invite a hosted bot:
- Invite the hosted bot with [this invite](http://bit.ly/44vrEev)
### Extend a hosted bot:
- For extensions of the audio clips, create either issue or directly a pull request.
- Adding new assets is just literally adding an audio file (ideally mp3) to the assets folder.
- Each file in the assets folder becomes its own command.
- It is also possible to group assets into subfolders. The subfolders become their own command and the assets inside become command options.
- To extend greetings (you want to map an asset file to a specific user greeting), please create an issue.
### Self-Host with completely custom assets and greetings:
- Clone the repository
- You need to extend the .env file. Please check .env.template, format the JSON, and see how to configure it. In the .env file, there has to be again minified JSON without spaces or newlines.
- Run the program with cargo or dockerize it however you want and deploy.
