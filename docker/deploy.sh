email=$1
testing=$2

if ! [[ "$testing" =~ ^[0-9]+$ ]]; then
	echo "You've given a testing flag that isn't a number, try again: $testing"
	exit 1
fi

if [[ "$testing" -ne '0' && "$testing" -ne '1' ]] 2> /dev/null; then
	echo "You've given a testing flag that isn't a 0 or a 1, try again: $testing"
	exit 1
fi

if [[ "$email" =~ ^[0-9]+$ ]]; then
	echo "You've given an email that looks like the 'testing' flag, try again: $email"
	exit 1
fi

# sting data found for $domains. Continue and replace existing certificate? (y/N) " decision
# # if [ "$decision" != "Y" ] && [ "$decision" != "y" ]; then
# # 	exit
# # fi


if [ ! -z "$email" ]; then
	echo "provided email: $email"
fi

if [ "$testing" -eq '0' ]; then
	echo "in testing mode"
else
	echo 'in live mode!!'
	# read -p "Existing data found for $domains. Continue and replace existing certificate? (y/N) " decision
	# if [ "$decision" != "Y" ] && [ "$decision" != "y" ]; then
	# 	exit
	# fi
fi

# if email isn't set and not testing
if [ -z "$email" ] && [ "$testing" -eq "0" ]; then
	echo "You haven't provided an email to be registered as the admin of your TLS certificate, and you're in live mode. try again"
	exit 1
fi

if [ ! -z "$email" ] && [ "$testing" -eq '1' ]; then
	echo "Email is set but in testing mode. Ignoring email."
fi

