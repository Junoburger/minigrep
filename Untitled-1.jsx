const BankSearch = ({ banks, searchCategory, setFilteredBanks }) => {
	const [searchString, setSearchString] = useState();

	const searchBanks = (search) => {
		const filteredBanks = [];
		banks.forEach((bank) => {
			if (bank[searchCategory].toLowerCase().includes(search.toLowerCase())) {
				console.log(bank[searchCategory].toLowerCase());
				filteredBanks.push(bank);
			}
		});

		setFilteredBanks(filteredBanks);
	};

	const debounceSearch = useCallback(_debounce(searchBanks, 500), []);

	useEffect(() => {
		if (searchString?.length) {
			debounceSearch(searchString);
		} else setFilteredBanks([]);
	}, [searchString, searchCategory]);

	const handleSearch = (e) => {
		setSearchString(e.target.value);
	};

	return (
		<div className="flex">
			<Input placeholder="Bank Search" onChange={handleSearch} />
		</div>
	);
};

export default BankSearch;
