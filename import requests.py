import requests

# Replace these with your actual API details
API_BASE_URL = "https://api.aviator.co"  # Base URL for the game's API
API_ENDPOINT = "/predicted-odds"  # Specific endpoint for fetching odds
API_KEY = "your_api_key_here"  # Replace with your actual API key

def get_predicted_odds():
    """
    Fetches the predicted odds from the game's server via its API.
    """
    url = f"{API_BASE_URL}{API_ENDPOINT}"
    headers = {
        "Authorization": f"Bearer {API_KEY}",
        "Content-Type": "application/json"
    }
    try:
        response = requests.get(url, headers=headers)
        if response.status_code == 200:
            odds_data = response.json()
            print("Predicted Odds:", odds_data)
            return odds_data
        else:
            print(f"Error: Unable to fetch odds (Status Code: {response.status_code})")
            print("Details:", response.text)
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    get_predicted_odds()
