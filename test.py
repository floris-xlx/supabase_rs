
import os
from supabase import create_client, Client
from dotenv import load_dotenv

load_dotenv()  # take environment variables from .env.


url: str = os.environ.get("SUPABASE_URL")
key: str = os.environ.get("SUPABASE_KEY")
supabase: Client = create_client(url, key)


def cooking() -> None:
    print("Cooking...")
    response = supabase.table('trades')\
        .select('id', count='exact')\
        .execute()
    
    print(response)


cooking()