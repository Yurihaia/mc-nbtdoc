#!/usr/bin/env python3
# Execute in the `scripts` directory.

from urllib.request import urlopen
from os.path import abspath
import subprocess
import json
import re
import sys

versionsUrl = 'https://meta.fabricmc.net/v1/versions'
mavenUrl = 'https://maven.fabricmc.net/net/fabricmc/fabric-api/fabric-api/maven-metadata.xml'

# Fetch versions.
with urlopen(versionsUrl) as url:
	versions: dict = json.loads(url.read().decode())

# Get and check game version.
gameVersion = versions['game'][0]['version']
print(f'gameVersion = {gameVersion}')

gitTags = subprocess.run(['git', 'tag'], stdout=subprocess.PIPE).stdout.decode('utf-8')
if (f'{gameVersion}-candidate' in gitTags) or (f'{gameVersion}-gen' in gitTags):
	print(f'{gameVersion} already exists in the git tag list.\nTask ends.')
	sys.exit()

print(f'{gameVersion} is an unknown version.\nStart generating.')

# Get mappings version and loader version.
mappingsVersion = next(filter(lambda d: d['gameVersion'] == gameVersion, versions['mappings']))['version']
loaderVersion = versions['loader'][0]['version']
print(f'mappingsVersion = {mappingsVersion}')
print(f'loaderVersion = {loaderVersion}')

# Get Fabric API version.
with urlopen(mavenUrl) as url:
	fabricVersion = re.search('<latest>(.*)</latest>', url.read().decode(), re.IGNORECASE).group(1)
print(f'fabricVersion = {fabricVersion}')

# Replace paths in `HijackBootstrap.java`.
hijackBootstrapPath = abspath('./nbtdoc-genmod/src/main/java/yurihaia/rd/mixin/HijackBootstrap.java')
mappingsFilePath = abspath('../generate/mappings.json')
generatedDirPath = abspath('../minecraft/generated')
print(f'mappingsFilePath = {mappingsFilePath}')
print(f'generatedDirPath = {generatedDirPath}')
print(f'Start configuring paths in {hijackBootstrapPath}.')
with open(hijackBootstrapPath, 'r+') as file:
	content = file.read()
	file.seek(0)
	file.write(content
		.replace('%MAPPINGS_FILE_PATH%', mappingsFilePath)
		.replace('%GENERATED_DIR_PATH%', generatedDirPath)
	)
	file.truncate()
print(f'Configured paths in {hijackBootstrapPath}.')

# Replace versions in `gradle.properties`.
gradlePropertiesPath = abspath('./nbtdoc-genmod/gradle.properties')
print(f'Start configuring versions in {gradlePropertiesPath}.')
with open(gradlePropertiesPath, 'r+') as file:
	content = file.read()
	file.seek(0)
	file.write(content
		.replace('%GAME_VERSION%', gameVersion)
		.replace('%MAPPINGS_VERSION%', mappingsVersion)
		.replace('%LOADER_VERSION%', loaderVersion)
		.replace('%FABRIC_VERSION%', fabricVersion)
	)
	file.truncate()
print(f'Configured versions in {gradlePropertiesPath}')

# Run Minecraft server.
print('Start running Minecraft server.')
returnCode = subprocess.run(['./gradlew', 'runServer'], cwd=abspath('./nbtdoc-genmod')).returncode
if returnCode == 0:
	print('Minecraft server exited.')
else:
	print(f'Minecraft server exited with error code {returnCode}\nTask ends.')
	sys.exit(1)

# Commit, push, and tag.
subprocess.run('git config user.name actions-user'.split(' '))
subprocess.run('git config user.email action@github.com'.split(' '))
subprocess.run('git add ..'.split(' '))
subprocess.run(['git', 'commit', '-m', f'Update to {gameVersion}'])
subprocess.run('git push'.split(' '))
subprocess.run(['git', 'tag', f'{gameVersion}-candidate'])
subprocess.run('git push --tags'.split(' '))
