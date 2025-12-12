# import os

# print("--- Environment Variables ---")
# for key, value in os.environ.items():
#     print(f"{key}: {value}")
# print("-----------------------------")

import os

# Access the standard environment variable we just created
root_path = os.environ.get('WORKSPACE_ROOT')

if root_path:
    print(f"Successfully retrieved root path: {root_path}")
    # Example usage:
    # with open(os.path.join(root_path, 'config.json'), 'r') as f:
    #     ...
else:
    print("Environment variable WORKSPACE_ROOT not set in the terminal environment.")
