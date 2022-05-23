# About
This project aims to produce a simple CLI program to fetch and display the headlines of the day.
Headlines can then be selected by the user, at which point the corresponding story will be downloaded and displayed.
Extra features may also include downloading a weather summary, and being able to store user preferences.

# Methodology
Data is accquired using newsapi.org and potential users will need to register to get an API key for it.

# Config
The app uses a config.json file to store the API key and other settings. A template is auto-generated if one is not present, but the template can also be seen below:

{
  "ApiKey":"*the user API key goes here*",
  "Source":"*the desired news source goes here; default is bbc-news*",
  "NumberOfHeadlines":"*the number of headlines to be displayed; default is 10*",
  "DisplayFormat":"*how much information is displayed: h for just headlines, h&d for headline and description, h&d&u for headline, description, url (default)*"
}
