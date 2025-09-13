export function Header() {
    return (
        <div className="flex justify-between items-center h-10">
            <div className="text-xl p-2">Rankit</div>
            <div>
                <button>About</button>
                <button>Derp</button>
                <button>Flurp</button>
            </div>
            <div>
                <button>Login</button>
                <button>Sign Up</button>
            </div>
        </div>
    );
}